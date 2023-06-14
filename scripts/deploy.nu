#!/usr/bin/env nu

# let ip_addr = '192.168.0.103'

def main [
  --init            # Init the cluster
  --hubble          # Enable hubble ui (service map)
  --debug (-v)      # Output to stdout instead of /dev/null
  --delete-pod (-d): string
] {
  if $delete_pod != $nothing {
    delete_pod $delete_pod
  }
}


def delete_pod [pod_name: string] {
  kubectl -n civilization get pods -l $"app=($pod_name)" -o jsonpath='{.items[0].metadata.name}' |
  kubectl delete -n civilization pod $in
}