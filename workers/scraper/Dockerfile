FROM python:3.12-slim-bookworm

# Copy the project into the image
ADD . /app

# Sync the project into a new environment, using the frozen lockfile
WORKDIR /app
RUN pip install uv && uv sync --frozen

# CMD to run the application
CMD ["uv", "run", "python", "src/main.py"]
