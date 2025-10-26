# User Management API

A production-ready REST API for user management with JWT authentication, built with Rust, Axum, and PostgreSQL.

## Features

- **User Registration & Authentication** - Secure user signup and login with JWT tokens
- **Password Security** - Bcrypt hashing with salting
- **Protected Routes** - JWT middleware for authorized endpoints
- **User Management** - Full CRUD operations for user profiles
- **Database Persistence** - PostgreSQL with connection pooling
- **Containerized Deployment** - Docker and Kubernetes ready

## Tech Stack

- **Rust** with Axum web framework
- **PostgreSQL** database with SQLx
- **JWT** authentication (24-hour token expiration)
- **Docker** for containerization
- **Kubernetes (k3s)** for orchestration

## API Endpoints

### Public Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/register` | Create a new user account |
| `POST` | `/login` | Authenticate and receive JWT token |
| `GET` | `/users` | List all users (passwords excluded) |
| `GET` | `/user/{id}` | Get user details by ID |

### Protected Endpoints (Require JWT)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `PUT` | `/user/{id}` | Update user profile |
| `DELETE` | `/user/{id}` | Delete user account |

**Authentication**: Include JWT in the `Authorization` header as `Bearer <token>`

## Quick Start (Docker Compose)

For local development:

```bash
docker-compose -f docker/docker-compose.yml up
```

Access the API at `http://localhost:3000`

## Kubernetes Deployment

### Prerequisites

```bash
# Install required packages (Arch Linux)
sudo pacman -S docker k3s kubectl

# Add user to docker group
sudo usermod -aG docker $USER
newgrp docker

# Verify docker
docker ps
```

### 1. Setup k3s

```bash
# Start k3s service
sudo systemctl start k3s
sudo systemctl enable k3s

# Configure kubectl
mkdir -p ~/.kube
sudo cp /etc/rancher/k3s/k3s.yaml ~/.kube/config
sudo chown $USER ~/.kube/config

# Verify cluster
kubectl get nodes
```

### 2. Build and Load Image

```bash
# Build Docker image
docker build -t api:latest .

# Import into k3s
docker save api:latest -o api.tar
sudo k3s ctr images import api.tar

# Verify
sudo k3s ctr images ls | grep api
```

### 3. Deploy Database

```bash
# Create namespace and ConfigMap
kubectl apply -f deployment/db/db-namespace.yaml
kubectl create configmap init-sql \
  --from-file=init.sql=./docker/init.sql \
  -n db-namespace

# Deploy PostgreSQL
kubectl apply -f deployment/db/db-deployment.yaml -n db-namespace
kubectl apply -f deployment/db/db-service.yaml -n db-namespace

# Wait for ready status
kubectl get pods -n db-namespace -w  # Ctrl+C when Running
```

**Verify database setup:**

```bash
# Check tables were created
kubectl exec -it $(kubectl get pod -l app=postgres -n db-namespace -o jsonpath='{.items[0].metadata.name}') \
  -n db-namespace -- psql -U postgres -d info_db -c "\dt"
```

### 4. Configure Application

Get the database service IP:

```bash
kubectl get service postgres-service -n db-namespace
```

Update `deployment/app/app-deployment.yaml` with the `CLUSTER-IP`:

```yaml
env:
  - name: DATABASE_URL
    value: postgresql://postgres:admin123@<CLUSTER-IP>:5432/info_db
```

### 5. Deploy Application

```bash
# Create namespace and deploy app
kubectl apply -f deployment/app/app-namespace.yaml
kubectl apply -f deployment/app/app-deployment.yaml -n app-namespace
kubectl apply -f deployment/app/app-service.yaml -n app-namespace

# Wait for ready status
kubectl get pods -n app-namespace -w  # Ctrl+C when Running

# Check logs
kubectl logs -l app=rust-app -n app-namespace
```

## Access the API

### Local Access

```bash
curl http://localhost:30000
```

### Network Access

From other machines on your network:

```bash
# Get your machine's IP
ip route get 1.2.3.4 | awk '{print $7}' | head -n1

# Access from another machine
curl http://<MACHINE_IP>:30000
```

## Configuration

Environment variables are managed in `conf/secrets.env`:

- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT signing

## Project Structure

```
api/
├── src/
│   ├── main.rs              # Application entry & routes
│   ├── model.rs             # Data structures
│   ├── auth.rs              # JWT & password utilities
│   ├── auth_controller.rs   # Auth handlers
│   ├── controller.rs        # User CRUD handlers
│   ├── user_service.rs      # Database operations
│   └── middleware.rs        # JWT validation
├── deployment/
│   ├── app/                 # Kubernetes manifests (API)
│   └── db/                  # Kubernetes manifests (PostgreSQL)
├── docker/
│   ├── docker-compose.yml   # Local development setup
│   └── init.sql             # Database schema
└── conf/
    └── secrets.env          # Configuration
```
