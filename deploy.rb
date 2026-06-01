#! /usr/bin/env ruby
# frozen_string_literal: true
require 'date'

# Remember to login to ghcr.io before running this script

surfix = ARGV[0] || ""
version = DateTime.now.strftime("%Y.%m.%d")
provider = "ghcr.io"
registry = "#{provider}/kelvinchincc"
tag = "pdf-convertor-bot"
platform = 'linux/amd64,linux/arm64'
target_tag = ["#{registry}/#{tag}:latest", "#{registry}/#{tag}:#{version}#{surfix == "" ? "" : "-#{surfix}"}"]
docker_build_command = "docker buildx build --platform #{platform} -t #{target_tag.join(" -t ")} . --push"

exec "#{docker_build_command}"
