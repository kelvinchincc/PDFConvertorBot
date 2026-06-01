#! /usr/bin/env ruby
# frozen_string_literal: true

require 'dotenv/load'

token = ENV['TELOXIDE_TOKEN']
secret_token = ENV['TELOXIDE_SECRET_TOKEN']
webhook_url = ENV['TELOXIDE_WEBHOOK_URL']

puts "Setting up webhook with URL: #{webhook_url} and secret token: #{secret_token}"

api = "https://api.telegram.org/bot#{token}/setWebhook"
payload = "{\"url\": \"#{webhook_url}\", \"secret_token\": \"#{secret_token}\"}"

exec "curl -X POST #{api} -H \"Content-Type: Application/json\" -d '#{payload}'"
