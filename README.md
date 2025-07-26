docker build -t gossip-glomers:latest .
docker run -it --rm --name app gossip-glomers
docker exec -it gossip-glomers sh