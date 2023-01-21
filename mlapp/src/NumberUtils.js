

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

    /**
     * Returns a hash code from a string
     *
     * I found this. Hope it works...
     *
     * @param  {String} str The string to hash.
     * @return {Number}    A 32bit integer
     * @see http://werxltd.com/wp/2010/05/13/javascript-implementation-of-javas-string-hashcode-method/
     */
    static hashCode(str) {
        let hash = 0;
        for (let i = 0, len = str.length; i < len; i++) {
            let chr = str.charCodeAt(i);
            hash = (hash << 5) - hash + chr;
            hash |= 0; // Convert to 32bit integer
        }
        return hash;
    }
}
