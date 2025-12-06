#pragma once
#include "BaseShadowdBlock.hpp"
#include "ShadowdBlock.hpp"

//capa 2
using SmallShadowdBlock = ShadowdBlock<bSB, EbSB>;
using sSB = SmallShadowdBlock;
using EntrySmallShadowdBlock = EntryShadowdBlock<2, sSB>;
using EsSB = EntrySmallShadowdBlock;

//capa 3
using MediumShadowdBlock = ShadowdBlock<sSB, EsSB>;
using mSB = MediumShadowdBlock;
using EntryMediumShadowdBlock = EntryShadowdBlock<3, mSB>;
using EmSB = EntryMediumShadowdBlock;

//capa 4 
using GreadShadowdBlock = ShadowdBlock<mSB, EmSB>;
using gSB = GreadShadowdBlock;
using EntryGreadShadowdBlock = EntryShadowdBlock<4, gSB>;
using EgSB = EntryGreadShadowdBlock;

//capa 5
using LargeShadowdBlock = ShadowdBlock<gSB, EgSB>;
using lSB = LargeShadowdBlock;
using EntryLargeShadowdBlock = EntryShadowdBlock<5, lSB>;
using ElSB = EntryLargeShadowdBlock;