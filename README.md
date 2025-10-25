# Install Docker, k3s, and kubectl
```
sudo pacman -S docker k3s kubectl


# Add your user to the docker group
sudo usermod -aG docker $USER
newgrp docker

# Verify docker works without sudo
docker ps

# Start k3s service
sudo systemctl start k3s
sudo systemctl enable k3s  # Auto-start on boot

# Set up kubectl configuration
mkdir -p ~/.kube
sudo cp /etc/rancher/k3s/k3s.yaml ~/.kube/config
sudo chown $USER ~/.kube/config

# Verify k3s is running
kubectl get nodes

# Build the Docker image
docker build -t api:latest .

# Save image to tar file
docker save api:latest -o api.tar

# Import into k3s
sudo k3s ctr images import api.tar

# Verify image is loaded
sudo k3s ctr images ls | grep api

# Create database initialization ConfigMap
kubectl create configmap init-sql \
  --from-file=init.sql=./docker/init.sql \
  -n db-namespace

# Deploy database resources
kubectl apply -f deployment/db/db-namespace.yaml
kubectl apply -f deployment/db/db-deployment.yaml -n db-namespace
kubectl apply -f deployment/db/db-service.yaml -n db-namespace

# Wait for database to be ready (Ctrl+C when STATUS shows "Running")
kubectl get pods -n db-namespace -w

# Verify database tables were created
kubectl exec -it $(kubectl get pod -l app=postgres -n db-namespace -o jsonpath='{.items[0].metadata.name}') \
  -n db-namespace -- psql -U postgres -d info_db -c "\dt"

# Get the database service IP
kubectl get service postgres-service -n db-namespace

# Update deployment/app/app-deployment.yaml with the CLUSTER-IP:
# env:
#   - name: DATABASE_URL
#     value: postgresql://postgres:admin123@<CLUSTER-IP>:5432/info_db

# Deploy application resources
kubectl apply -f deployment/app/app-namespace.yaml
kubectl apply -f deployment/app/app-deployment.yaml -n app-namespace
kubectl apply -f deployment/app/app-service.yaml -n app-namespace

# Wait for application to be ready (Ctrl+C when STATUS shows "Running")
kubectl get pods -n app-namespace -w

# Check application logs
kubectl logs -l app=rust-app -n app-namespace
```
# Local Access
curl http://localhost:30000

# Network Access (other machines)
Get your machine's network IP
ip route get 1.2.3.4 | awk '{print $7}' | head -n1
http://<MACHINE_IP>:30000
