#!/usr/bin/env nu

let ip_addr = '192.168.0.103'

def main [
  --init            # Init the cluster and CNI
  --hubble          # Enable hubble ui (service map)
  --debug (-v)    # Output to stdout instead of /dev/null
] {
  if $init {
    sudo kubeadm init $'--apiserver-advertise-address=($ip_addr)' $'--apiserver-cert-extra-sans=($ip_addr)' 

    mkdir $'($env.HOME)/.kube'
    sudo cp -i /etc/kubernetes/admin.conf $'($env.HOME)/.kube/config'
    sudo chown $'(id -u):(id -g)' $'($env.HOME)/.kube/config'

    cilium install
  } else if $hubble {
    cilium hubble enable --ui
    # kubectl apply -f ../configs/hubble_ui_service.yaml
    # print $'Service map is avaliable on http://($ip_addr):12000'
  }
}