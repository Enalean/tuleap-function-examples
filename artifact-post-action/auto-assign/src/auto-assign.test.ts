/**
 * MIT License
 *
 * Copyright (c) 2024 Enalean
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

import { describe, it, expect } from "vitest";
import { autoAssign } from "./auto-assign";
import type { ArtifactChange } from "./auto-assign";

const CONTRIBUTOR_FIELD_ID = 12;
const USER_ID_DOING_THE_CHANGE = 102;
const EXPECTED_VALUE_NO_CHANGE = {values: []};

describe("auto-assign", () => {
    it ("automatically assigns artifact creator as contributor", () => {
        expect(autoAssign(createArtifactChangeNeedingAContributor())).toStrictEqual(
            {values: [{field_id: CONTRIBUTOR_FIELD_ID, bind_value_ids: [USER_ID_DOING_THE_CHANGE]}]}
        );
    });

    it ("does nothing on update", () => {
        const artifact_change_update: ArtifactChange = {
            ...createArtifactChangeNeedingAContributor(),
            action: "update",
        }
        expect(autoAssign(artifact_change_update)).toStrictEqual(EXPECTED_VALUE_NO_CHANGE);
    });


    it ("does nothing when a contributor has already been set", () => {
        let artifact_change_with_contributor = createArtifactChangeNeedingAContributor();
        artifact_change_with_contributor.current.values[0] = {
            field_id: CONTRIBUTOR_FIELD_ID,
            bind_value_ids: [789]
        };
        expect(autoAssign(artifact_change_with_contributor)).toStrictEqual(EXPECTED_VALUE_NO_CHANGE);
    });

    it ("throws an error when tracker does not have a contributor semantic", () => {
        let artifact_change_no_contributor_semantic = createArtifactChangeNeedingAContributor();
        delete artifact_change_no_contributor_semantic.tracker.semantics.contributor;
        expect(() => autoAssign(artifact_change_no_contributor_semantic)).toThrowErrorMatchingInlineSnapshot(`[Error: The tracker does not have the contributor/assignee semantic set]`);
    });

    it ("throws an error when tracker have a contributor semantic that is not a selectbox", () => {
        let artifact_change_with_broken_contributor_semantic = createArtifactChangeNeedingAContributor();
        delete artifact_change_with_broken_contributor_semantic.current.values[0].bind_value_ids;
        expect(() => autoAssign(artifact_change_with_broken_contributor_semantic)).toThrowErrorMatchingInlineSnapshot(`[Error: The contributor/assignee field does not seem to be a selectbox]`);
    });
});

function createArtifactChangeNeedingAContributor(): ArtifactChange {
    return {
        action: "create",
        current: {
            submitted_by: USER_ID_DOING_THE_CHANGE,
            values: [
                {
                    field_id: CONTRIBUTOR_FIELD_ID,
                    bind_value_ids: [100]
                }
            ],
        },
        tracker: {
            semantics: {
                contributor: {
                    field_id: CONTRIBUTOR_FIELD_ID,
                }
            }
        }
    }
}