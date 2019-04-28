function isLeapYear(year: number): boolean {

    if (year % 4 === 0) {
        if (year % 100 === 0) {
            return (year % 400 === 0)
        } else {
            return true
        }
    } else {
        return false
    }

}

export default isLeapYear