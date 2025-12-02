import * as v from "valibot";
import { BaseObjectSchema } from "./base_object";

export const SubjectSchema = v.object({
    ...BaseObjectSchema.entries,
    data: v.object({
        characters: v.nullish(v.string()),
        character_images: v.nullish(
            v.array(
                v.object({
                    url: v.string(),
                    content_type: v.string(),
                })
            )
        ),
        meanings: v.array(
            v.object({
                meaning: v.string(),
                primary: v.boolean(),
                accepted_answer: v.boolean(),
            })
        ),
        readings: v.nullish(
            v.array(
                v.object({
                    reading: v.string(),
                    primary: v.boolean(),
                    accepted_answer: v.boolean(),
                    type_: v.nullish(v.string()),
                })
            )
        ),
        level: v.number(),
        slug: v.string(),
        component_subject_ids: v.nullish(v.array(v.number())),
        amalgamation_subject_ids: v.nullish(v.array(v.number())),
        visually_similar_subject_ids: v.nullish(v.array(v.number())),
        meaning_mnemonic: v.nullish(v.string()),
        meaning_hint: v.nullish(v.string()),
        reading_mnemonic: v.nullish(v.string()),
        reading_hint: v.nullish(v.string()),
    }),
    review_data: v.nullish(v.object({
        incorrect_meaning_answers: v.pipe(v.number(), v.minValue(0)),
        incorrect_reading_answers: v.pipe(v.number(), v.minValue(0)),
    }))
})

export type Subject = v.InferOutput<typeof SubjectSchema>;