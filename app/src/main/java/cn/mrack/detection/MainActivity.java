/**
 * @author Mrack
 * @date 2022/12/13
 */
package cn.mrack.detection;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends Activity {
    static {
        System.loadLibrary("rust");
    }

    private TextView libcTv;
    private TextView libArtTv;

    private native String check(String s);

    @SuppressLint("SetTextI18n")
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        libcTv = findViewById(R.id.libc);
        libArtTv = findViewById(R.id.libart);

        new Thread(() -> {
            while (true) {
                String libcResult = check("libc.so");
                String libArtResult = check("libart.so");
                runOnUiThread(() -> {
                    libcTv.setTextColor(libcResult.contains("not") ? 0xff00ff00 : 0xffff0000);
                    libArtTv.setTextColor(libArtResult.contains("not") ? 0xff00ff00 : 0xffff0000);
                    libcTv.setText("libc " + libcResult);
                    libArtTv.setText("libart " + libArtResult);
                });
                try {
                    Thread.sleep(1500);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }
        }).start();
    }
}