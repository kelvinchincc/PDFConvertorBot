#! /usr/bin/env ruby
# frozen_string_literal: true
require 'date'

surfix = ARGV[0] || ""
version = DateTime.now.strftime("%Y.%m.%d")
provider = "codeberg.org"
registry = "#{provider}/kelvinchincc"
tag = "pdf-convertor-bot"
platform = 'linux/amd64,linux/arm64'
target_tag = ["#{registry}/#{tag}:latest", "#{registry}/#{tag}:#{version}#{surfix == "" ? "" : "-#{surfix}"}"]
docker_login_command = "docker login #{provider}"
docker_build_command = "docker buildx build --platform #{platform} -t #{target_tag.join(" -t ")} . --push"

exec "docker logout ; #{docker_login_command} && #{docker_build_command}"
