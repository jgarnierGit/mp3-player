use std::rc::Rc;

use audio_player::AudioTag;

#[derive(Debug, PartialEq, Clone)]
pub enum TagAggregatorResult {
    Value {
        tag: AudioTag,
        count: Rc<usize>,
        child: Box<TagAggregatorResult>,
    },
    Nil,
}

/// Builds an hierarchical TagAggregatorResult based on AudioTags list
///
/// # Arguments
///  * `tags` - AudioTag list
///
pub fn build_structure(tags: &Vec<AudioTag>) -> Box<TagAggregatorResult> {
    let mut tag_aggregator_model = TagAggregatorResult::Nil;
    for tag in tags.iter().rev() {
        let val = TagAggregatorResult::Value {
            tag: *tag,
            count: Rc::new(0),
            child: Box::new(tag_aggregator_model),
        };
        tag_aggregator_model = val;
    }
    Box::new(tag_aggregator_model)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_aggr_struct_with_child(
        node: Box<TagAggregatorResult>,
        check_audio_tag: AudioTag,
        check_count: usize,
        child_tag: AudioTag,
    ) -> Result<(), String> {
        let node_content = *node;
        match node_content.clone() {
            TagAggregatorResult::Value { tag, count, child } => {
                check_audio_tag_node(tag, check_audio_tag)?;
                check_count_node(*count.as_ref(), check_count)?;
                // check child is as expected
                let child_to_check = *child;
                match child_to_check {
                    TagAggregatorResult::Value {
                        tag: child_tag_value,
                        ..
                    } => {
                        if child_tag_value != child_tag {
                            return Err(format!(
                                "Expected Tag child node of '{:?}' to be node of '{:?}' type, got {:?}",
                                tag, child_tag, child_to_check
                            ));
                        }
                    }
                    TagAggregatorResult::Nil => {
                        return Err(format!(
                            "Expected a child Node here for {:?} ",
                            check_audio_tag
                        ));
                    }
                }
            }
            TagAggregatorResult::Nil => {
                return Err(format!("Expected a Node here for {:?} ", check_audio_tag));
            }
        }
        Ok(())
    }

    fn check_aggr_struct_leaf(
        node: Box<TagAggregatorResult>,
        check_audio_tag: AudioTag,
        check_count: usize,
    ) -> Result<(), String> {
        let node_content = *node;
        match node_content.clone() {
            TagAggregatorResult::Value { tag, count, child } => {
                check_audio_tag_node(tag, check_audio_tag)?;
                check_count_node(*count.as_ref(), check_count)?;
                // check child is as expected
                let child_to_check = *child;
                if child_to_check != TagAggregatorResult::Nil {
                    return Err(format!(
                        "Expected Tag child node of '{:?}' to be {:?}, got {:?}",
                        tag,
                        TagAggregatorResult::Nil,
                        child_to_check
                    ));
                }
            }
            TagAggregatorResult::Nil => {
                return Err(format!("Expected a Node here for {:?} ", check_audio_tag));
            }
        }
        Ok(())
    }

    fn check_aggr_struct_nil(node: Box<TagAggregatorResult>) -> Result<(), String> {
        let node_content = *node;
        match node_content.clone() {
            TagAggregatorResult::Value {
                tag: _,
                count: _,
                child: _,
            } => {
                return Err(format!("Expected a Nil value, not {:?} ", node_content));
            }
            TagAggregatorResult::Nil => {}
        }
        Ok(())
    }

    fn check_audio_tag_node(tag: AudioTag, check_audio_tag: AudioTag) -> Result<(), String> {
        if check_audio_tag != tag {
            return Err(format!(
                "Expected {:?} Tag for this node, got {:?}",
                check_audio_tag, tag
            ));
        }
        Ok(())
    }

    fn check_count_node(count: usize, check_count: usize) -> Result<(), String> {
        if count != check_count {
            return Err(format!(
                "Expected Tag count to be {}, got {}",
                check_count, count
            ));
        }
        Ok(())
    }

    #[test]
    fn it_build_aggregator_struct_one_level() -> Result<(), String> {
        let tags = vec![AudioTag::Genre];
        let aggr_struct = build_structure(&tags);
        check_aggr_struct_leaf(aggr_struct.clone(), AudioTag::Genre, 0)?;
        if let TagAggregatorResult::Value {
            tag: _,
            count: _,
            child,
        } = *aggr_struct
        {
            check_aggr_struct_nil(child)?;
        }
        return Ok(());
    }

    #[test]
    fn it_build_aggregator_struct_many_level() -> Result<(), String> {
        let tags = vec![AudioTag::Genre, AudioTag::Artist, AudioTag::Album];
        let aggr_struct = build_structure(&tags);
        // checking the root
        check_aggr_struct_with_child(aggr_struct.clone(), AudioTag::Genre, 0, AudioTag::Artist)?;
        //checking first child
        if let TagAggregatorResult::Value {
            tag: _,
            count: _,
            child,
        } = *aggr_struct
        {
            check_aggr_struct_with_child(child.clone(), AudioTag::Artist, 0, AudioTag::Album)?;
            let first_child = *child;
            // checking the leaf
            if let TagAggregatorResult::Value {
                tag: _,
                count: _,
                child,
            } = first_child
            {
                check_aggr_struct_leaf(child.clone(), AudioTag::Album, 0)?;
            }
        }

        return Ok(());
    }
}
