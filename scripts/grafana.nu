#!/usr/bin/nu

let grafana_dir = '../configs/grafana'
cat $'($grafana_dir)/deployment.yaml' | envsubst | kubectl apply -f -
kubectl apply -f $'($grafana_dir)/grafana-datasource-config.yaml'
kubectl apply -f $'($grafana_dir)/service.yaml'