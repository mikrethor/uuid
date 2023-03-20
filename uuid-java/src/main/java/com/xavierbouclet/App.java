package com.xavierbouclet;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.UUID;

public class App {

    public static void main(String[] args) throws Exception {
        var filepath = args[0];
        var count = Long.parseLong(args[1]);
        ObjectMapper objectMapper = new ObjectMapper();

        objectMapper.readValue(Files.readString(Paths.get(filepath)), new TypeReference<List<UUID>>() {
        }).stream().limit(count).forEach(
                System.out::println
        );
    }
}