package com.xavierbouclet;

import com.fasterxml.jackson.databind.ObjectMapper;

import java.net.ProxySelector;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.UUID;

public class App {

    public static void main(String[] args) throws Exception {
        var version = args[0];
        var count = args[1];
        HttpRequest request = HttpRequest.newBuilder()
                .uri(new URI("https://www.uuidtools.com/api/generate/v%s/count/%s".formatted(version, count)))
                .GET()
                .build();

        var response = HttpClient
                .newBuilder()
                .proxy(ProxySelector.getDefault())
                .build()
                .send(request, HttpResponse.BodyHandlers.ofString())
                .body();

        ObjectMapper objectMapper = new ObjectMapper();
        UUID[] uuids = objectMapper.readValue(response, UUID[].class);
        for (UUID uuid : uuids) {
            System.out.println(uuid);
        }
    }
}
