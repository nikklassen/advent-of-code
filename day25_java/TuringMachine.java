import java.util.ArrayList;
import java.util.List;

public class TuringMachine {
    private class Operation {
        int write;
        int direction;
        States nextState;
        Operation(int write, int direction, States nextState) {
            this.write = write;
            this.direction = direction;
            this.nextState = nextState;
        }
    }
    private class State {
        Operation op0;
        Operation op1;
        State(Operation op0, Operation op1) {
            this.op0 = op0;
            this.op1 = op1;
        }
        public Operation getOperation(int value) {
            return value == 0 ? this.op0 : this.op1;
        }
    }

    private enum States {
        A, B, C, D, E, F
    }

    private State[] states;
    private State currentState;
    private ArrayList<Integer> tape;
    private int pos;

    TuringMachine() {
        this.states = new State[] {
                // A
                new State(new Operation(1, 1, States.B), new Operation(1, -1, States.E)),
                // B
                new State(new Operation(1, 1, States.C), new Operation(1, 1, States.F)),
                // C
                new State(new Operation(1, -1, States.D), new Operation(0, 1, States.B)),
                // D
                new State(new Operation(1, 1, States.E), new Operation(0, -1, States.C)),
                // E
                new State(new Operation(1, -1, States.A), new Operation(0, 1, States.D)),
                // F
                new State(new Operation(1, 1, States.A), new Operation(1, 1, States.C)),
        };
        this.currentState = this.states[0];
        this.tape = new ArrayList<>();
        this.tape.add(0);
        this.pos = 0;
    }

    public List<Integer> getTape() {
        return new ArrayList<>(this.tape);
    }

    public void next() {
        Operation op = this.currentState.getOperation(this.tape.get(this.pos));
        this.tape.set(this.pos, op.write);
        this.move(op.direction);
        this.currentState = this.states[op.nextState.ordinal()];
    }

    private void move(int direction) {
        if (direction == -1 && this.pos == 0) {
            this.tape.add(0, 0);
            return;
        } else if (direction == 1 && this.pos == this.tape.size() - 1) {
            this.tape.add(0);
        }
        this.pos += direction;
    }
}
