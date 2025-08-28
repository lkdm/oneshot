FROM ghcr.io/astral-sh/uv:alpine

WORKDIR /workspace

# Create a Python virtual environment at /opt/venv using uv
RUN uv venv /opt/venv

# Set environment variables so the virtualenv is activated by default
ENV VIRTUAL_ENV=/opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Use a shell entrypoint so oneshot can run commands interactively
ENTRYPOINT ["/bin/sh"]
