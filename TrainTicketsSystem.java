import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.HashMap;
import java.util.Map;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        int n = Integer.parseInt(reader.readLine());
        HashMap<String, Double> prices = new HashMap<>();
        HashMap<String, Double> profit = new HashMap<>();
        for (int i = 0; i < n; i++){
            String input = reader.readLine();
            String[] data = input.substring(0, input.length() - 3).split(" ");
            prices.put(data[0], Double.parseDouble(data[1]));
        }
        for (Map.Entry<String, Double> entry : prices.entrySet()){
            String k = entry.getKey();
            double v = entry.getValue();
            System.out.println(k + ":");
            String input = reader.readLine();
            int i = 0;
            double profitPerCity = 0.0;
            while (!input.equals("NEXT DESTINATION")){
                String ageStr = "";
                for (char c : input.toCharArray()){
                    if (c == 'y'){
                        break;
                    }
                    ageStr += c;
                }
                int age = Integer.parseInt(ageStr);
                int time = Integer.parseInt(input.substring(input.length() - 5, input.length()-3));
                double percantage = 0.0;
                if ((time >= 0 && time < 7) || (time >= 19 && time <= 23)){
                    percantage += 0.05;
                }
                if (age >= 7 && age < 12){
                    percantage += 0.2;
                } else if (age >= 12 && age < 18 || age > 64){
                    percantage += 0.1;
                }
                double price = v;
                double discount = price*percantage;
                price -= discount;
                if (age < 7){
                    price = 0;
                    i--;
                }
                profitPerCity += price;
                i++;
                input = reader.readLine();
            }
            if (i >= 4){
                profitPerCity -= prices.get(k) * 0.05 * i;
            }
            profit.put(k, profitPerCity);
        }
        profit.forEach((k, v) -> {
            System.out.println(k + " " + v);
        });
    }
}
