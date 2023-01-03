

export default class NumberUtils {

    static clamp(x, lower, upper) {
        if (x > upper) {
            return upper;
        } else if (x < lower) {
            return lower
        } else {
            return x;
        }
    }

    static randRange(lower, upper) {
        return (Math.random() * (upper - lower)) + lower;
    }
}
