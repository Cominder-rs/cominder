#!/usr/bin/nu

kubectl create namespace ingress-controller
helm install haproxy-ingress haproxy-ingress/haproxy-ingress --namespace ingress-controller --version 0.14.2 --set controller.service.type=NodePort