#!/usr/bin/env bash

aws s3 cp "$(pwd)/../dist" s3://jun.codes/ --recursive
