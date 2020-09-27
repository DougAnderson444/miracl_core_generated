/*
 * Copyright (c) 2012-2020 MIRACL UK Ltd.
 *
 * This file is part of MIRACL Core
 * (see https://github.com/miracl/core).
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
pub mod arch;
pub mod aes;
pub mod gcm;
pub mod hmac;
pub mod hash256;
pub mod hash384;
pub mod hash512;
pub mod rand;
pub mod sha3;
pub mod nhs;

#[cfg(feature = "ed25519")]
pub mod ed25519;
#[cfg(feature = "c25519")]
pub mod c25519;
#[cfg(feature = "nist256")]
pub mod nist256;
#[cfg(feature = "brainpool")]
pub mod brainpool;
#[cfg(feature = "anssi")]
pub mod anssi;
#[cfg(feature = "hifive")]
pub mod hifive;
#[cfg(feature = "goldilocks")]
pub mod goldilocks;
#[cfg(feature = "nist384")]
pub mod nist384;
#[cfg(feature = "c41417")]
pub mod c41417;
#[cfg(feature = "nist521")]
pub mod nist521;
#[cfg(feature = "nums256w")]
pub mod nums256w;
#[cfg(feature = "nums256e")]
pub mod nums256e;
#[cfg(feature = "nums384w")]
pub mod nums384w;
#[cfg(feature = "nums384e")]
pub mod nums384e;
#[cfg(feature = "nums512w")]
pub mod nums512w;
#[cfg(feature = "nums512e")]
pub mod nums512e;
#[cfg(feature = "secp256k1")]
pub mod secp256k1;
#[cfg(feature = "sm2")]
pub mod sm2;
#[cfg(feature = "c13318")]
pub mod c13318;
#[cfg(feature = "jubjub")]
pub mod jubjub;
#[cfg(feature = "x448")]
pub mod x448;
#[cfg(feature = "secp160r1")]
pub mod secp160r1;
#[cfg(feature = "c1174")]
pub mod c1174;
#[cfg(feature = "c1665")]
pub mod c1665;
#[cfg(feature = "mdc")]
pub mod mdc;
#[cfg(feature = "tweedledee")]
pub mod tweedledee;
#[cfg(feature = "tweedledum")]
pub mod tweedledum;
#[cfg(feature = "bn254")]
pub mod bn254;
#[cfg(feature = "bn254CX")]
pub mod bn254CX;
#[cfg(feature = "bls12383")]
pub mod bls12383;
#[cfg(feature = "bls12381")]
pub mod bls12381;
#[cfg(feature = "fp256bn")]
pub mod fp256bn;
#[cfg(feature = "fp512bn")]
pub mod fp512bn;
#[cfg(feature = "bls12461")]
pub mod bls12461;
#[cfg(feature = "bn462")]
pub mod bn462;
#[cfg(feature = "bls24479")]
pub mod bls24479;
#[cfg(feature = "bls48556")]
pub mod bls48556;
#[cfg(feature = "bls48581")]
pub mod bls48581;
#[cfg(feature = "bls48286")]
pub mod bls48286;
#[cfg(feature = "rsa2048")]
pub mod rsa2048;
#[cfg(feature = "rsa3072")]
pub mod rsa3072;
#[cfg(feature = "rsa4096")]
pub mod rsa4096;
