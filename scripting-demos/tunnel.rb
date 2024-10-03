require 'json'

def lambda_handler(event:, context:)
  message = event['message'].to_s.strip.downcase

  response = if message == 'echo'
               ['dark ECHO', 'light ECHO'].sample
             else
               "You said: #{message}. Say 'echo' to play the tunnel game!"
             end

  {
    statusCode: 200,
    body: response
  }
end
