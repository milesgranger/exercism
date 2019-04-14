
interface NumberParse {
    number: number | null,
    offset: number
}

class RunLengthEncoding {
    static encode(msg: string): string {

        // Set init vars
        let encoded: string = '';
        let currentChar: string = '';
        let count: number = 1;

        // Begin encoding
        for (let i = 0; i <= msg.length; i++) {

            // First iteration, set initial char
            if (i === 0) {
                currentChar = msg[i];
            }

            // There is a match, increment count
            else if (currentChar === msg[i]) {
                count += 1;
            }

            // No match, currentSegment should be appended to encoded
            else {
                encoded += `${encoded}${count > 1 ? count : ''}${currentChar}`;
                currentChar = msg[i];
                count = 1;
            }
        }

        return encoded
    }

    static first_integer(msg: string): NumberParse {
        let number_str: string = '';

        for (let i = 0; i <= msg.length; i ++) {
            if (isNaN(parseInt(msg[i]))) {
                return {number: number_str.length > 0 ? parseInt(number_str) : null, offset: i}
            }
            else {
                number_str += msg[i]
            }
        }
    }

    static decode(msg: string): string {

        let decoded: string = '';

        while (msg.length > 1) {

            let currentDecoded: NumberParse = RunLengthEncoding.first_integer(msg);
            msg = msg.slice(0, currentDecoded.offset);


        }
    }
}

export default RunLengthEncoding