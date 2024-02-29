package me.underworld.dante.helpers;

import android.content.res.AssetManager;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Iterator;
import java.util.Locale;
import java.util.Map;
import org.json.JSONObject;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

import me.underworld.dante.DanteGlobal;

public class LangDict {
    public static String filename;
    public static final Map stringSet = new HashMap();

    public static void loadStringsFromFile(String language) {
        if(language == null){
            String lang = Locale.getDefault().getLanguage();
            if(lang.equals("en")){
                language = "en";
            } else if (lang.equals("tr")) {
                language = "tr";
            }else{
                language = "en";
            }

        }
        try{
            AssetManager assets = DanteGlobal.resources.getAssets();
            InputStream open = assets.open("i18n/" + language + ".json");
            byte[] bArr = new byte[open.available()];
            open.read(bArr);
            open.close();
            JSONObject jSONObject = new JSONObject(new String(bArr, "UTF-8"));
            Iterator keys = jSONObject.keys();
            while (keys.hasNext()) {
                String str = (String) keys.next();
                stringSet.put(str, jSONObject.getString(str));
            }
        } catch (Exception e) {

        }

    }

    public static String getStringValue(String key) {
        String str2 = (String) stringSet.get(key);
        return (str2 != null || stringSet.containsKey(key)) ? str2 : "??";
    }
}
