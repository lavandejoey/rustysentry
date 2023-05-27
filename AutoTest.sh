#!/usr/bin/env sh
exec_path=./target/debug/rustysentry
debug_log() {
  echo "[$(date +%Y-%m-%d\ %H:%M:%S)]-$1-$2"
}
debug_log "INFO" "AutoTest.sh start"
############################## 1. build ##############################
debug_log "INFO" "build start"
#cargo build
#if [ $? -ne 0 ]; then
#  debug_log "ERROR" "build failed"
#  exit 1
#fi
debug_log "INFO" "build end"
############################## 2. run ##############################
#debug_log "INFO" "run start"
#$exec_path
#if [ $? -ne 0 ]; then
#  debug_log "ERROR" "run failed"
#  exit 1
#fi
#debug_log "INFO" "run end"
############################## 3. test ##############################
debug_log "INFO" "test start"
output=$($exec_path --version)
if [ $? -ne 0 ]; then
  debug_log "ERROR" "version flag failed"
  exit 1
fi
debug_log "INFO" "version flag success"
output=$($exec_path --help)
if [ $? -ne 0 ]; then
  debug_log "ERROR" "help flag failed"
  exit 1
fi
debug_log "INFO" "help flag success"
output=$($exec_path --config ./config.toml)
if [ $? -ne 0 ]; then
  debug_log "ERROR" "config flag failed"
  exit 1
fi
debug_log "INFO" "config flag success"


debug_log "INFO" "AutoTest.sh end"