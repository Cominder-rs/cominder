#!/usr/bin/env nu

let ip_addr = '192.168.0.103'

sudo kubeadm init $'--apiserver-advertise-address=($ip_addr)' $'--apiserver-cert-extra-sans=($ip_addr)' 

mkdir $'($env.HOME)/.kube'
sudo cp -i /etc/kubernetes/admin.conf $'($env.HOME)/.kube/config'
sudo chown $'(id -u):(id -g)' $'($env.HOME)/.kube/config'

kubectl taint nodes --all node-role.kubernetes.io/control-plane-
cilium install
kubectl apply -f ../configs/namespace.yaml
./dockerhub-cred.nu