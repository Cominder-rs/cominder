#!/usr/bin/nu

let prometheus_dir = '../configs/prometheus'

kubectl apply -f $"($prometheus_dir)"