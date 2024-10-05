export const typeRelations = {
  Flora: {
    strong_against: ["Aqua", "Terra", "Boulder"],
    weak_against: ["Inferno", "Frost", "Bug", "Venom"],
  },
  Aqua: {
    strong_against: ["Inferno", "Terra", "Boulder"],
    weak_against: ["Flora", "Spark"],
  },
  Inferno: {
    strong_against: ["Flora", "Bug", "Frost", "Boulder"],
    weak_against: ["Aqua", "Terra", "Inferno"],
  },
  Zephyr: {
    strong_against: ["Flora", "Brawler", "Bug"],
    weak_against: ["Spark", "Boulder"],
  },
  Terra: {
    strong_against: ["Spark", "Metal", "Inferno", "Boulder"],
    weak_against: ["Aqua", "Flora", "Zephyr"],
  },
  Spark: {
    strong_against: ["Aqua", "Zephyr"],
    weak_against: ["Terra", "Boulder"],
  },
  Boulder: {
    strong_against: ["Zephyr", "Bug", "Frost"],
    weak_against: ["Aqua", "Flora", "Brawler", "Terra"],
  },
  Specter: { strong_against: ["Specter", "Psyche"],
     weak_against: ["Shadow"] },
  Metal: {
    strong_against: ["Frost", "Boulder"],
    weak_against: ["Inferno", "Brawler", "Venom"],
  },
  Psyche: {
    strong_against: ["Brawler", "Venom"],
    weak_against: ["Shadow", "Bug"],
  },
  Venom: {
    strong_against: ["Flora", "Brawler"],
    weak_against: ["Psyche", "Boulder"],
  },
  Brawler: {
    strong_against: ["Boulder", "Metal", "Shadow"],
    weak_against: ["Psyche", "Zephyr"],
  },
  Shadow: { strong_against: ["Psyche", "Specter"], weak_against: ["Brawler"] },
  Frost: {
    strong_against: ["Flora", "Zephyr", "Mystic"],
    weak_against: ["Inferno", "Metal", "Boulder"],
  },
  Mystic: { strong_against: ["Mystic"], weak_against: ["Frost"] },
  Bug: {
    strong_against: ["Flora", "Psyche", "Venom"],
    weak_against: ["Inferno", "Zephyr", "Boulder"],
  },
  Neutral: { strong_against: [], weak_against: ["Brawler"] },
};
