module GitHubHelpers

using HTTP, JSON3

export GITHUB_RETRY, headers, github_get, github_get_bytes, github_get_json

const headers = if haskey(ENV, "GITHUB_TOKEN")
    Dict("Authorization" => "token " * ENV["GITHUB_TOKEN"])
else
    nothing
end

const GITHUB_RETRY = (; retries=5, initial_delay=1.0, max_delay=30.0)

function response_body_string(resp)
    body = resp.body
    if isnothing(body)
        return ""
    elseif body isa AbstractVector{UInt8}
        return String(copy(body))
    else
        return sprint(show, body)
    end
end

function http_response_error(url, resp)
    header_lines = ["$(name): $(value)" for (name, value) in resp.headers]
    body = response_body_string(resp)

    parts = [
        "GitHub request failed for $url",
        "HTTP $(resp.status)",
    ]

    isempty(header_lines) || push!(parts, join(header_lines, "\n"))
    isempty(body) || push!(parts, body)

    ErrorException(join(parts, "\n\n"))
end

function github_get(url; retry=GITHUB_RETRY)
    delay = retry.initial_delay
    last_error = nothing

    for attempt in 1:retry.retries
        try
            resp = HTTP.get(url; headers=headers, status_exception=false)
            if 200 <= resp.status < 300
                return resp
            end
            response_error = http_response_error(url, resp)
            if resp.status in (403, 429) || 500 <= resp.status < 600
                last_error = response_error
            else
                throw(response_error)
            end
        catch err
            last_error = err
        end

        if attempt < retry.retries
            sleep(delay)
            delay = min(delay * 2, retry.max_delay)
        end
    end

    throw(last_error)
end

github_get_bytes(url; retry=GITHUB_RETRY) = github_get(url; retry=retry).body
github_get_json(url; retry=GITHUB_RETRY) = JSON3.read(github_get_bytes(url; retry=retry))

end
