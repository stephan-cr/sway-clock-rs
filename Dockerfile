FROM scratch
COPY target/release/sway-clock /
CMD ["/sway-clock", "%Y-%m-%d %l:%M:%S %p"]
