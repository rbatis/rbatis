## 该镜像为一次性镜像，无法挂起，容器运行即停。压测部署在8000的服务
```cmd
docker run -it --rm williamyeh/wrk -t12 -c400 -d30s http://0.0.0.0:8000
```