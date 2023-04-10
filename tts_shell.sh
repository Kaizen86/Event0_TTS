#!/bin/bash

while [ true ]; do
  echo -n "\$ "
  read command
  $command 2>&1 | ./target/debug/event0_tts
done
