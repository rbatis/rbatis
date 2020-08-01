
```cmd
docker pull williamyeh/wrk 该镜像为一次性镜像，无法挂起，容器运行即停。
docker run -it --rm williamyeh/wrk -t12 -c400 -d30s http://192.168.28.105:8000 压测部署在8000的服务
```