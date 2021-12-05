#!/bin/bash
set +e
cat picks | while read id; do echo ./script.sh $id; done
