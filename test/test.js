/*
 * SPDX-FileCopyrightText: 2024 Eduardo Javier Alvarado Aarón <eduardo.javier.alvarado.aaron@gmail.com>
 *
 * SPDX-License-Identifier: Unlicense
 */

let quoted_labeled = // XML label
	"<tag a=${va} b='${va}'>text \" ${va}</tag>";
let quoted_normal = "<tag a=${va} b='${va}'>text \" ${va}</tag>";

let quoted = /*XML*/ "<tag a=${va} b='${va}'>text \" ${va}</tag>";
let normal_quoted = "<tag a=${va} b='${va}'>text \" ${va}</tag>";
let quoted_multi = /* XML label */ "\
	<tag a=${va} b='${va}'>text \" ${va}</tag>\
";

let literal_labeled = // XML label
	`<tag a=${va} b='${va}'>text \" ${va}</tag>`;
let literal_normal = `<tag a=${va} b='${va}'>text \" ${va}</tag>`;

let literal = /*XML*/ `<tag a=${va} b='${va}'>text \" ${va}</tag>`;
let normal_literal  = `<tag a=${va} b='${va}'>text \" ${va}</tag>`;
let literal_multi = /* XML label */ `
	<tag a=${va} b='${va}'>text \" ${va}</tag>
`;
