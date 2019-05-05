module SpaceAge
open System


let EARTH_SECONDS = float 31557600
let MERCURY_SECS = 0.2408467 * EARTH_SECONDS
let VENUS_SECS = 0.61519726 * EARTH_SECONDS
let MARS_SECS = 1.8808158 * EARTH_SECONDS
let JUPITER_SECS = 11.862615 * EARTH_SECONDS
let SATURN_SECS = 29.447498 * EARTH_SECONDS
let URANUS_SECS = 84.016846 * EARTH_SECONDS
let NEPTUNE_SECS = 164.79132 * EARTH_SECONDS

type Planet =
    | Earth
    | Mercury
    | Venus
    | Mars
    | Jupiter
    | Saturn
    | Uranus
    | Neptune

let age (planet: Planet) (seconds: int64): float =
    match planet with
    | Planet.Earth -> float seconds / EARTH_SECONDS
    | Planet.Mercury -> float seconds / MERCURY_SECS
    | Planet.Venus -> float seconds / VENUS_SECS
    | Planet.Mars -> float seconds / MARS_SECS
    | Planet.Jupiter -> float seconds / JUPITER_SECS
    | Planet.Saturn -> float seconds / SATURN_SECS
    | Planet.Uranus -> float seconds / URANUS_SECS
    | Planet.Neptune -> float seconds / NEPTUNE_SECS
    