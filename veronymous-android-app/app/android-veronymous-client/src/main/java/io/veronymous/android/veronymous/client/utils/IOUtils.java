package io.veronymous.android.veronymous.client.utils;

import android.content.Context;
import android.util.Log;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;

import io.veronymous.android.veronymous.client.exceptions.VeronymousIOException;

public class IOUtils {

    private static final String TAG = IOUtils.class.getSimpleName();

    private IOUtils() {
        throw new IllegalStateException("Utility class.");
    }

    public static String readString(Context context, File file) throws VeronymousIOException {
        try (FileInputStream inputStream = context.openFileInput(file.getName())) {
            InputStreamReader inputStreamReader
                    = new InputStreamReader(inputStream, StandardCharsets.UTF_8);

            StringBuilder stringBuilder = new StringBuilder();

            try (BufferedReader reader = new BufferedReader(inputStreamReader)) {
                String line = reader.readLine();

                while (line != null) {
                    stringBuilder.append(line).append('\n');
                    line = reader.readLine();
                }
            } catch (IOException e) {
                throw new VeronymousIOException(e);
            }

            return stringBuilder.toString();
        } catch (IOException e) {
            throw new VeronymousIOException(e);
        }
    }

    public static void writeString(Context context, File file, String contents)
            throws VeronymousIOException {
        try (FileOutputStream outputStream = context.openFileOutput(file.getName(), Context.MODE_PRIVATE)) {
            outputStream.write(contents.getBytes(StandardCharsets.UTF_8));
        } catch (IOException e) {
            throw new VeronymousIOException(e);
        }
    }
}
