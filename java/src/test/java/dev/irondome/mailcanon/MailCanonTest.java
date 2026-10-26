package dev.irondome.mailcanon;

import static org.junit.jupiter.api.Assertions.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HexFormat;
import java.util.Locale;
import java.util.stream.Stream;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

final class MailCanonTest {
    @TestFactory Stream<DynamicTest> strictVectors() throws Exception {
        return Files.readAllLines(Path.of("..", "testdata", "strict-vectors.tsv")).stream()
            .filter(row -> !row.startsWith("#")).map(row -> {
                String[] v = row.split("\t", -1);
                return DynamicTest.dynamicTest("strict: " + v[0], () -> {
                    String raw = new String(HexFormat.of().parseHex(v[1]), StandardCharsets.UTF_8);
                    if (v[2].equals("-")) {
                        assertThrows(MailCanon.InvalidEmailException.class, () -> MailCanon.normalizeStrict(raw));
                        assertThrows(MailCanon.InvalidEmailException.class, () -> MailCanon.canonicalizeStrict(raw));
                    } else {
                        assertEquals(new MailCanon.Identity(v[2], v[3], v[4]), MailCanon.normalizeStrict(raw));
                        assertEquals(v[3], MailCanon.canonicalizeStrict(raw));
                        assertEquals(v[3], MailCanon.canonicalizeStrict(v[3]));
                    }
                });
            });
    }

    @TestFactory Stream<DynamicTest> vectors() throws Exception {
        return Files.readAllLines(Path.of("..", "testdata", "vectors.tsv")).stream()
            .filter(row -> !row.startsWith("#")).map(row -> {
                String[] v = row.split("\t", -1);
                return DynamicTest.dynamicTest(v[0], () -> {
                    String raw = new String(HexFormat.of().parseHex(v[1]), StandardCharsets.UTF_8);
                    if (v[2].equals("-")) {
                        assertThrows(MailCanon.InvalidEmailException.class, () -> MailCanon.normalize(raw));
                        assertThrows(MailCanon.InvalidEmailException.class, () -> MailCanon.canonicalize(raw));
                    } else {
                        assertEquals(new MailCanon.Identity(v[2], v[3], v[4]), MailCanon.normalize(raw));
                        assertEquals(v[3], MailCanon.canonicalize(raw));
                        assertEquals(v[3], MailCanon.canonicalize(v[3]));
                    }
                });
            });
    }

    @Test void nullAndInvalidUtf16() {
        assertThrows(NullPointerException.class, () -> MailCanon.normalize(null));
        assertThrows(NullPointerException.class, () -> MailCanon.canonicalize(null));
        assertThrows(NullPointerException.class, () -> MailCanon.normalizeStrict(null));
        assertThrows(NullPointerException.class, () -> MailCanon.canonicalizeStrict(null));
        for (String raw : new String[]{"\ud800@example.com", "name \udfff <u@example.com>"})
            assertThrows(MailCanon.InvalidEmailException.class, () -> MailCanon.normalize(raw));
    }

    @Test void localeDoesNotChangeAsciiCasing() {
        Locale previous = Locale.getDefault();
        try {
            Locale.setDefault(Locale.forLanguageTag("tr-TR"));
            assertEquals("identity@gmail.com", MailCanon.canonicalize("IDENTITY@GMAIL.COM"));
        } finally { Locale.setDefault(previous); }
    }

    @Test void protonAndFastmailRules() {
        var proton = MailCanon.normalizeStrict("User.Name_-+extra@protonmail.ch");
        assertEquals("proton", proton.provider());
        assertEquals("protonmail.ch", proton.domain());
        assertEquals("username@protonmail.ch", proton.canonical());
        assertEquals(proton.canonical(), MailCanon.canonicalizeStrict(proton.canonical()));

        var direct = MailCanon.normalizeStrict("User.Name+tag@fastmail.com");
        assertEquals("fastmail", direct.provider());
        assertEquals("fastmail.com", direct.domain());
        assertEquals("user.name@fastmail.com", direct.canonical());

        var alias = MailCanon.normalizeStrict("tag@User-Name.fastmail.com.au");
        assertEquals("fastmail", alias.provider());
        assertEquals("user-name.fastmail.com.au", alias.domain());
        assertEquals("user-name@fastmail.com.au", alias.canonical());
        assertEquals(alias.canonical(), MailCanon.canonicalizeStrict(alias.canonical()));
        assertEquals("custom", MailCanon.normalizeStrict("tag@sub.user.fastmail.com").provider());
        assertEquals("custom", MailCanon.normalizeStrict("tag@user.fastmail.example").provider());
        assertThrows(MailCanon.InvalidEmailException.class, () -> MailCanon.normalizeStrict("+tag@proton.me"));
    }
}
