import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.Arrays;

public class Main {
    static void swap(String[] nums, int l, int i) {
        String temp = nums[l];
        nums[l] = nums[i];
        nums[i] = temp;
    }

    static void permutations(ArrayList<String[]> res, String[] nums, int l, int h) {
        if (l == h) {
            res.add(Arrays.copyOf(nums, nums.length));
            return;
        }
        for (int i = l; i <= h; i++) {
            swap(nums, l, i);
            permutations(res, nums, l + 1, h);
            swap(nums, l, i);
        }
    }

    static ArrayList<String[]> permute(String[] nums) {
        ArrayList<String[]> res = new ArrayList<>();
        int x = nums.length - 1;
        permutations(res, nums, 0, x);
        return res;
    }

    public static void main(String[] args) throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        String input = reader.readLine();
        int originalNumber = Integer.parseInt(input);
        //int[] number = Arrays.stream(input.split("")).mapToInt(Integer::parseInt).toArray();
        String[] number = input.split("");
        boolean isDragon = false;
        boolean isPseudoDragon = false;
        if (originalNumber >= 100) {
            ArrayList<String[]> permutations = permute(number);
            for (int i = 0; i < permutations.size(); i++) {
                String num = String.join("", permutations.get(i));
                int leftNum = Integer.parseInt(num.substring(0, num.length() / 2));
                int rightNum = Integer.parseInt(num.substring(num.length() / 2));
                if (num.length() % 2 == 0) {
                    if (leftNum * rightNum == originalNumber) {
                        isDragon = true;
                    }
                } else if (num.length() > 2) {
                    if (i % 2 == 0) {
                        leftNum = Integer.parseInt(num.substring(0, num.length() / 2 + 1));
                        rightNum = Integer.parseInt(num.substring(num.length() / 2 + 1));
                    }
                    if (leftNum * rightNum == originalNumber) {
                        isPseudoDragon = true;
                    }
                }
            }
        }
        if (isDragon) {
            System.out.println("True Dragon");
        } else if (isPseudoDragon) {
            System.out.println("Pseudodragon");
        } else {
            System.out.println("Normal number");
        }
    }
}