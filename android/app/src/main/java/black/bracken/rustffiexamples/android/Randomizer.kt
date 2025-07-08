package black.bracken.rustffiexamples.android

object Randomizer {
    init {
        System.loadLibrary("randomizer")
    }

    external fun genRandomNumber(number: Int): Int
}