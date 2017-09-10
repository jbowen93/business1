cargo watch --ignore-nothing -w target/x86_64-unknown-linux-musl/release -s 'docker build -f Deploy-Dockerfile -t jbowen/test-rocket:latest .; docker push jbowen/test-rocket:latest; kubectl delete deploy rocket-cd; kubectl run rocket-cd --image=jbowen/rocket-test:latest'