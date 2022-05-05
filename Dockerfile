FROM centos:centos7

RUN yum update -y && yum install -y gcc \
 && rm -rf /var/cache/yum/* \
 && yum clean all

RUN curl -o rustup.rs --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
 && sh rustup.rs -y \
 && rm -f rustup.rs 

ENV PATH=$PATH:/root/.cargo/bin
