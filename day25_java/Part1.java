import java.util.List;
import java.util.stream.Collectors;

public class Part1 {
    public static void main(String[] args) {
        TuringMachine m = new TuringMachine();
        for (int i = 0; i < 12459852; i++) {
            m.next();
        }
        List<Integer> tape = m.getTape();
        int count = tape.stream().collect(Collectors.summingInt(Integer::intValue));
        System.out.println(count);
    }
}