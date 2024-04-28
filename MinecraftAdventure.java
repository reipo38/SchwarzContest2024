import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
    public static void main(String[] args) throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        String input = reader.readLine();
        int numSticks = 0, numStone = 0;
        while (!input.equals("END")){
            if (input.equals("Sticks")){
                numSticks++;
            }
            if (input.equals("Wood")) {
                numSticks += 4;
            }
            if (input.equals("Cobblestone")) {
                numStone++;
            }
            input = reader.readLine();
        }
        int amount = 0;
        if (numSticks >= 2 && numStone >= 3){
            numSticks /= 2;
            numStone /= 3;
            amount = Math.min(numSticks, numStone);
        }
        System.out.println("Amount of stone pickaxes: " + amount);
    }
}