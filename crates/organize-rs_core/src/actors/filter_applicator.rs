use crate::{actors::location_walker::DirEntryData, filters::FilterGroupCollection};
use itertools::{Either, Itertools};

use crate::filters::{
    FilterApplicationKind, FilterFilterClosureSliceMut, FilterGroup, FilterGroupOperationKind,
    FilterKind,
};

#[derive(Debug, Default)]
pub struct FilterApplicator {
    filters: FilterGroupCollection,
}

impl FilterApplicator {
    pub fn new(filters: FilterGroupCollection) -> Self {
        FilterApplicator { filters }
    }

    pub fn get_applicable_items(self, entries: DirEntryData) -> DirEntryData {
        // extract ignore filters
        let (ignore_filters, other_filters): (Vec<_>, Vec<_>) =
            self.filters
                .iter()
                .partition_map(|filter| match filter.mode() {
                    FilterApplicationKind::None => Either::Left(filter),
                    _ => Either::Right(filter),
                });

        // split off any / all filters
        let (any_filters, all_filters): (Vec<_>, Vec<_>) =
            other_filters
                .into_iter()
                .partition_map(|filter| match filter.mode() {
                    FilterApplicationKind::Any => Either::Left(filter),
                    FilterApplicationKind::All => Either::Right(filter),
                    _ => unreachable!(
                        "There should be no items left in `FilterModeGroupKind::None`!"
                    ),
                });

        Self::get_filtered_entries(entries, ignore_filters, any_filters, all_filters)
    }

    pub fn apply_filters(
        entries: Vec<jwalk::DirEntry<((), ())>>,
        filters: FilterFilterClosureSliceMut<((), ())>,
    ) -> Vec<jwalk::DirEntry<((), ())>> {
        entries
            .into_iter()
            .filter(|entry| {
                let mut results = vec![];
                filters
                    .iter_mut()
                    .for_each(|filter| results.push(filter(entry)));
                results.contains(&true)
            })
            .collect_vec()
    }

    fn filter_group_applies(
        filter_group: &FilterGroup<Vec<FilterKind>>,
        entry: &jwalk::DirEntry<((), ())>,
    ) -> bool {
        let (matched, not_matched): (Vec<bool>, Vec<bool>) = filter_group
            .filters()
            .iter()
            .map(|single_filter| single_filter.get_filter()(entry))
            .partition(|f| *f);

        match (filter_group.apply(), filter_group.mode()) {
            (FilterGroupOperationKind::Exclude, FilterApplicationKind::All)
            | (FilterGroupOperationKind::Include, FilterApplicationKind::All) => {
                not_matched.is_empty()
            }
            (FilterGroupOperationKind::Exclude, FilterApplicationKind::Any)
            | (FilterGroupOperationKind::Include, FilterApplicationKind::Any) => {
                !matched.is_empty()
            }
            (FilterGroupOperationKind::Exclude, FilterApplicationKind::None)
            | (FilterGroupOperationKind::Include, FilterApplicationKind::None) => {
                matched.is_empty()
            }
        }
    }

    fn get_filtered_entries(
        entries: DirEntryData,
        ignore_filters: Vec<&FilterGroup<Vec<FilterKind>>>,
        any_filters: Vec<&FilterGroup<Vec<FilterKind>>>,
        all_filters: Vec<&FilterGroup<Vec<FilterKind>>>,
    ) -> DirEntryData {
        let filtered_entries = entries
            .into_iter()
            .flat_map(|entry| {
                if !ignore_filters.is_empty() {
                    ignore_filters
                        .iter()
                        // .inspect(|f| println!("Applying ignore filter: {f}"))
                        .map(|filter| Self::filter_group_applies(filter, &entry))
                        .all(|f| matches!(f, false))
                        .then_some(entry)
                } else {
                    Some(entry)
                }
            })
            .flat_map(|entry| {
                if !all_filters.is_empty() {
                    all_filters
                        .iter()
                        // .inspect(|f| println!("Applying filter: {f}"))
                        .map(|filter| Self::filter_group_applies(filter, &entry))
                        .all(|f| matches!(f, true))
                        .then_some(entry)
                } else {
                    Some(entry)
                }
            })
            .flat_map(|entry| {
                if !any_filters.is_empty() {
                    any_filters
                        .iter()
                        // .inspect(|f| println!("Applying filter: {f}"))
                        .map(|filter| Self::filter_group_applies(filter, &entry))
                        .any(|f| matches!(f, true))
                        .then_some(entry)
                } else {
                    Some(entry)
                }
            })
            .collect_vec();

        DirEntryData::from(filtered_entries)
    }
}
