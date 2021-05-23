# Use the container that comes with `cross` as a base. It's already got
# a cross-compile toolchain installed, so that's less work for us.
FROM rustembedded/cross:armv7-unknown-linux-gnueabihf

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes libasound2-dev:armhf

ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
