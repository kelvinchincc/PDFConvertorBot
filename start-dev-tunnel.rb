#! /usr/bin/env ruby
# frozen_string_literal: true

path = '/pdf-converter-bot'
port = 3000
exec "tailscale funnel --set-path #{path} http://localhost:#{port}#{path}/"
