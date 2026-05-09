module GitHubHelpers

using HTTP, JSON3
using GitHub

export GITHUB_RETRY, github_api_json, github_commits, github_get_bytes, github_gitcommit, github_tags, github_tree

const headers = if haskey(ENV, "GITHUB_TOKEN")
    Dict(
        "Authorization" => "token " * ENV["GITHUB_TOKEN"],
        "User-Agent" => "GitHub-jl",
    )
else
    Dict("User-Agent" => "GitHub-jl")
end

const GITHUB_RETRY = (; retries=5, initial_delay=1.0, verbose=false, max_sleep_seconds=30.0)
const GITHUB_API_HOST = "api.github.com"

github_auth() = haskey(ENV, "GITHUB_TOKEN") ? GitHub.OAuth2(ENV["GITHUB_TOKEN"]) : GitHub.AnonymousAuth()
github_repo(owner, repo) = GitHub.Repo("$owner/$repo")

function github_options(retry)
    (
        auth=github_auth(),
        max_retries=retry.retries,
        verbose=retry.verbose,
        max_sleep_seconds=retry.max_sleep_seconds,
    )
end

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

function github_api_get(path; params=Dict{String,Any}(), retry=GITHUB_RETRY)
    resp = GitHub.gh_get(
        GitHub.DEFAULT_API,
        path;
        github_options(retry)...,
        params=params,
        handle_error=false,
    )

    if 200 <= resp.status < 300
        return resp
    end

    throw(http_response_error(string(HTTP.URI(GitHub.api_uri(GitHub.DEFAULT_API, path), query=params)), resp))
end

function github_api_json(path; params=Dict{String,Any}(), retry=GITHUB_RETRY)
    JSON3.read(response_body_string(github_api_get(path; params=params, retry=retry)))
end

function github_commits(owner, repo; sha=nothing, per_page=nothing, page=nothing, retry=GITHUB_RETRY)
    params = Dict{String,Any}()
    isnothing(sha) || (params["sha"] = sha)
    isnothing(per_page) || (params["per_page"] = per_page)
    isnothing(page) || (params["page"] = page)
    commits, _ = GitHub.commits(github_repo(owner, repo); github_options(retry)..., params=params, page_limit=1)
    commits
end

function github_gitcommit(owner, repo, sha; retry=GITHUB_RETRY)
    GitHub.gitcommit(github_repo(owner, repo), sha; github_options(retry)...)
end

function github_tree(owner, repo, sha; recursive=false, retry=GITHUB_RETRY)
    params = recursive ? Dict{String,Any}("recursive" => "1") : Dict{String,Any}()
    GitHub.tree(github_repo(owner, repo), sha; github_options(retry)..., params=params)
end

function github_tags(owner, repo; retry=GITHUB_RETRY)
    tags, _ = GitHub.tags(github_repo(owner, repo); github_options(retry)...)
    tags
end

function raw_get(url; retry=GITHUB_RETRY)
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
            sleep(min(delay, retry.max_sleep_seconds))
            delay *= 2
        end
    end

    throw(last_error)
end

function github_get(url; retry=GITHUB_RETRY)
    uri = HTTP.URI(url)

    if uri.host == GITHUB_API_HOST
        return github_api_get(uri.path; params=Dict{String,Any}(HTTP.queryparams(uri)), retry=retry)
    end

    return raw_get(url; retry=retry)
end

github_get_bytes(url; retry=GITHUB_RETRY) = github_get(url; retry=retry).body
github_get_json(url; retry=GITHUB_RETRY) = JSON3.read(github_get_bytes(url; retry=retry))

end
