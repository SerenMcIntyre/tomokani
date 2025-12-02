import * as v from "valibot";

export const ReviewSchema = v.object({
    subject_id: v.nullish(v.number()),
    assignment_id: v.nullish(v.number()),
    incorrect_meaning_answers: v.number(),
    incorrect_reading_answers: v.number(),
    created_at: v.string(),
}); 