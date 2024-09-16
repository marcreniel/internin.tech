FROM rust:1.70 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

ARG TARGETARCH

COPY --from=builder /usr/src/app/target/release/ziptern-scraper /usr/local/bin/ziptern-scraper
COPY --from=builder /usr/src/app/src/.env /usr/local/bin/.env
COPY --from=builder /usr/src/app/src/constants /usr/local/bin/constants
COPY --from=builder /usr/src/app/src/output /usr/local/bin/output

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    wget \
    unzip \
    gpg \
    libglib2.0-0 \
    libnss3 \
    libfontconfig1 \
    libx11-6 \
    libxcb1 \
    libxcomposite1 \
    libxcursor1 \
    libxdamage1 \
    libxext6 \
    libxfixes3 \
    libxi6 \
    libxrandr2 \
    libxrender1 \
    libxtst6 \
    libasound2 \
    && rm -rf /var/lib/apt/lists/*

RUN if [ "$TARGETARCH" = "amd64" ]; then \
    wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_amd64.deb \
    && dpkg -i libssl1.1_1.1.1f-1ubuntu2_amd64.deb \
    && rm libssl1.1_1.1.1f-1ubuntu2_amd64.deb; \
elif [ "$TARGETARCH" = "arm64" ]; then \
    wget http://ports.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2_arm64.deb \
    && dpkg -i libssl1.1_1.1.1f-1ubuntu2_arm64.deb \
    && rm libssl1.1_1.1.1f-1ubuntu2_arm64.deb; \
fi

RUN GECKODRIVER_VERSION=$(wget -qO- https://api.github.com/repos/mozilla/geckodriver/releases/latest | grep tag_name | cut -d '"' -f 4) && \
    if [ "$TARGETARCH" = "amd64" ]; then \
        wget -q "https://github.com/mozilla/geckodriver/releases/download/${GECKODRIVER_VERSION}/geckodriver-${GECKODRIVER_VERSION}-linux64.tar.gz" && \
        tar -xzf geckodriver-${GECKODRIVER_VERSION}-linux64.tar.gz && \
        rm geckodriver-${GECKODRIVER_VERSION}-linux64.tar.gz; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
        wget -q "https://github.com/mozilla/geckodriver/releases/download/${GECKODRIVER_VERSION}/geckodriver-${GECKODRIVER_VERSION}-linux-aarch64.tar.gz" && \
        tar -xzf geckodriver-${GECKODRIVER_VERSION}-linux-aarch64.tar.gz && \
        rm geckodriver-${GECKODRIVER_VERSION}-linux-aarch64.tar.gz; \
    else \
        echo "Unsupported architecture: $TARGETARCH" && exit 1; \
    fi

RUN chmod +x /usr/local/bin/geckodriver

RUN install -d -m 0755 /etc/apt/keyrings && \
    wget -q https://packages.mozilla.org/apt/repo-signing-key.gpg -O- | gpg --dearmor > /etc/apt/keyrings/packages.mozilla.org.gpg && \
    echo "deb [signed-by=/etc/apt/keyrings/packages.mozilla.org.gpg] https://packages.mozilla.org/apt mozilla main" | tee /etc/apt/sources.list.d/mozilla.list > /dev/null && \
    echo 'Package: *\nPin: origin packages.mozilla.org\nPin-Priority: 1000\n\nPackage: firefox*\nPin: release o=Ubuntu\nPin-Priority: -1' | tee /etc/apt/preferences.d/mozilla > /dev/null && \
    apt-get update && \
    apt-get install -y firefox && \
    rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y xvfb

ENV DISPLAY=:99

CMD Xvfb :99 -ac & \
    geckodriver --port 4444 & \
    sleep 2 && \
    ziptern-scraper