#!/usr/bin/env python3
import argparse
import logging
import re
import subprocess

WORKLOADS = {
    "echo": {
        "m_workload": "echo",
        "m_args": ["--node-count", "1", "--time-limit", "20", "--rate", "100"],
        "ds_server": "echo",
    },
    "unique-ids": {
        "m_workload": "unique-ids",
        "m_args": [
            "--node-count",
            "3",
            "--time-limit",
            "30",
            "--rate",
            "1000",
            "--availability",
            "total",
            "--nemesis",
            "partition",
        ],
        "ds_server": "unique-id",
    },
    "broadcast1": {
        "m_workload": "broadcast",
        "m_args": ["--node-count", "1", "--time-limit", "20", "--rate", "10"],
        "ds_server": "broadcast",
    },
    "broadcast2": {
        "m_workload": "broadcast",
        "m_args": ["--node-count", "5", "--time-limit", "20", "--rate", "10"],
        "ds_server": "broadcast",
    },
}


def check_prereqs():
    is_ok = True

    if not check_installed("java"):
        logging.error("java is not installed")
        is_ok = False

    if not check_java_version(11):
        logging.error("java 11+ is not installed")
        is_ok = False

    if not check_installed("dot"):
        logging.error("graphviz (`dot`) is not installed")
        is_ok = False

    if not check_installed("gnuplot"):
        logging.error("gnuplot is not installed")
        is_ok = False

    return is_ok


def check_installed(cmd):
    try:
        result = subprocess.run(
            ["which", cmd], stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )
        return result.returncode == 0
    except Exception:
        logging.warning("`which` program not found")
        return False


def check_java_version(min_version_required):
    try:
        result = subprocess.run(
            ["java", "-version"], stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )
        output = result.stderr.decode("utf-8")
        version_match = re.match('java version "(\d+)\..*".*', output)

        if version_match:
            installed_java_version = int(version_match.group(1))
            if min_version_required <= installed_java_version:
                return True
            else:
                logging.warning(
                    f"installed java version {installed_java_version} < {min_version_required}"
                )
                return False
        else:
            logging.warning("unexpected output format for `java -version`")
            return False
    except Exception:
        logging.warning("java not installed")
        return False


def build_ds_sandbox(mode):
    if mode != "release" and mode != "debug":
        logging.warning(f"build mode {mode} must be either `release` or `debug`")
        mode = debug

    cmd = ["cargo", "build"]

    if mode == "release":
        cmd += ["--release"]

    result = subprocess.run(cmd)
    return result.returncode == 0


def get_path_to_ds_sandbox_bin(mode):
    return f"target/{mode}/main"


def run_maelstrom_test(
    workload_name, jar_path, maelstrom_args, ds_bin_path, ds_server, ds_args=[]
):
    # Run the maelstrom jar with Java.
    cmd = ["java", "-jar", jar_path]

    # Tell maelstrom to run a test with the given workload name.
    cmd += ["test", "-w", workload_name]

    # Pass the path to the ds_sandbox binary.
    cmd += ["--bin", ds_bin_path]

    # Add any additional maelstrom arguments needed for this test.
    cmd += maelstrom_args

    # Add any additional ds_sandbox arguments.
    cmd += ["--"]
    cmd += ds_args
    cmd += [ds_server]

    result = subprocess.run(cmd)
    return result.returncode == 0


def main():
    logging.basicConfig(level=logging.INFO)
    check_prereqs()

    # Parse arguments from command line.
    parser = argparse.ArgumentParser()

    parser.add_argument("workload")
    parser.add_argument(
        "-j", "--maelstrom", required=True, help="Path to maelstrom.jar"
    )
    parser.add_argument(
        "--mode",
        choices=["release", "debug"],
        default="debug",
        help="Compile mode when building",
    )
    parser.add_argument(
        "--log",
        choices=["Off", "Error", "Warn", "Info", "Debug", "Trace"],
        default="Warn",
        help="Minimum logging level",
    )

    args = parser.parse_args()

    # Ensure ds_sandbox is built
    logging.info(f"building ds_sandbox in {args.mode} mode")

    if not build_ds_sandbox(args.mode):
        logging.error("failed to build ds_sandbox")
        return 1

    if WORKLOADS[args.workload] is None:
        logging.error(f"unknown workload {args.workload}")
        return 1

    # Run the requested maelstrom test.
    run_maelstrom_test(
        WORKLOADS[args.workload]["m_workload"],
        args.maelstrom,
        WORKLOADS[args.workload]["m_args"],
        get_path_to_ds_sandbox_bin(args.mode),
        WORKLOADS[args.workload]["ds_server"],
        ["--log", args.log],
    )


if __name__ == "__main__":
    main()
